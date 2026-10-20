//! Signing a desktop app in to the person's Redrob account (redrob-auth, `https://auth.redrob.io`).
//!
//! OAuth 2.0 for native apps (RFC 8252): [`login`] opens the system browser on redrob-auth's
//! authorize page with PKCE (S256) and waits on a loopback listener (`127.0.0.1`, a free port) for
//! the redirect, checking `state`, for at most five minutes. [`login_with_device_code`] is the
//! RFC 8628 device grant for a computer whose browser can't reach the loopback. Each app signs in
//! itself under its own client id; tokens are never handed to another process.
//!
//! The refresh token lives in the OS keychain (service [`KEYCHAIN_SERVICE`], account = the client
//! id); the access token lives in memory only and [`Session::access_token`] refreshes it a minute
//! before it expires. When redrob-auth doesn't know the client or the grant, calls fail with
//! [`Error::NotAvailable`] and the caller keeps its existing Console key path.
//!
//! Every call blocks on the network: call it off the UI thread.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

/// redrob-auth in production.
pub const ISSUER: &str = "https://auth.redrob.io";
/// Overrides the issuer (staging); only an `https://` value is used.
pub const ISSUER_ENV: &str = "REDROB_AUTH_ISSUER";
/// The keychain service every Redrob app stores its refresh token under (account = client id).
pub const KEYCHAIN_SERVICE: &str = "ai.redrob.login";

const SCOPE: &str = "openid profile email";
const LOGIN_TIMEOUT: Duration = Duration::from_secs(300);
/// Refresh this long before the access token's `exp`.
const REFRESH_EARLY: u64 = 60;
/// Used when the token answer carries no `expires_in` and the token no `exp`.
const FALLBACK_LIFETIME: u64 = 300;
const DEVICE_GRANT: &str = "urn:ietf:params:oauth:grant-type:device_code";
const DEVICE_FALLBACK_EXPIRES: u64 = 600;
const DEVICE_MAX_EXPIRES: u64 = 1800;
const DEVICE_FALLBACK_INTERVAL: u64 = 5;
/// RFC 8628 3.5: `slow_down` adds five seconds to the interval.
const SLOW_DOWN: u64 = 5;
const MAX_UNREACHABLE: u32 = 5;
const MAX_BODY: u64 = 64 * 1024;
const MAX_REQUEST_LINE: u64 = 8 * 1024;

/// Why a sign-in call failed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// Sign-in with redrob-auth isn't offered for this client (unknown client, grant not allowed,
    /// no device endpoint, no discovery document). Keep using the existing Console key path.
    NotAvailable(String),
    /// No stored sign-in, or redrob-auth no longer accepts it (the refresh token was revoked or expired).
    NotSignedIn,
    /// The person declined on redrob-auth.
    Denied,
    /// The browser sign-in didn't come back within five minutes.
    TimedOut,
    /// The device code expired before it was approved.
    Expired,
    /// The redirect's `state` wasn't the one this sign-in sent.
    StateMismatch,
    /// The system browser couldn't be opened.
    Browser(String),
    /// redrob-auth couldn't be reached.
    Network(String),
    /// redrob-auth answered with an error.
    Server(String),
    /// The OS keychain refused.
    Keychain(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotAvailable(why) => write!(f, "Redrob sign-in isn't available here ({why})"),
            Error::NotSignedIn => write!(f, "not signed in to Redrob"),
            Error::Denied => write!(f, "the sign-in was declined"),
            Error::TimedOut => write!(f, "the sign-in timed out"),
            Error::Expired => write!(f, "the code expired"),
            Error::StateMismatch => write!(f, "the sign-in answer didn't match this sign-in"),
            Error::Browser(e) => write!(f, "couldn't open the browser ({e})"),
            Error::Network(e) => write!(f, "couldn't reach auth.redrob.io ({e})"),
            Error::Server(e) => write!(f, "auth.redrob.io refused ({e})"),
            Error::Keychain(e) => write!(f, "the keychain refused ({e})"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// What redrob-auth says about the signed-in person.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserInfo {
    /// The redrob-auth user id.
    pub sub: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

/// The code the person approves for [`login_with_device_code`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceCode {
    pub user_code: String,
    pub verification_uri: String,
    /// The approval page with the code filled in, when redrob-auth gives one.
    pub verification_uri_complete: Option<String>,
    pub expires_in: Duration,
}

/// The issuer: [`ISSUER_ENV`] when it is an `https://` URL, else [`ISSUER`].
pub fn issuer() -> String {
    std::env::var(ISSUER_ENV)
        .ok()
        .map(|s| s.trim().trim_end_matches('/').to_string())
        .filter(|s| s.starts_with("https://"))
        .unwrap_or_else(|| ISSUER.to_string())
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

fn random_b64(bytes: usize) -> Result<String> {
    let mut buf = vec![0u8; bytes];
    getrandom::fill(&mut buf).map_err(|e| Error::Server(format!("no randomness: {e}")))?;
    Ok(URL_SAFE_NO_PAD.encode(buf))
}

/// PKCE (RFC 7636): a 43-character verifier and its S256 challenge.
fn pkce() -> Result<(String, String)> {
    let verifier = random_b64(32)?;
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    Ok((verifier, challenge))
}

/// The `exp` claim of a JWT, read without verifying it (only to schedule a refresh).
fn jwt_exp(token: &str) -> Option<u64> {
    let payload = URL_SAFE_NO_PAD.decode(token.split('.').nth(1)?).ok()?;
    serde_json::from_slice::<Value>(&payload).ok()?.get("exp")?.as_u64()
}

// ---- HTTP -------------------------------------------------------------------------------------

/// HTTPS to redrob-auth, with the OS verifying its certificate. Never follows redirects.
struct Http(ureq::Agent);

impl Http {
    fn new() -> Self {
        let tls = ureq::tls::TlsConfig::builder().root_certs(ureq::tls::RootCerts::PlatformVerifier).build();
        Self(
            ureq::Agent::config_builder()
                .tls_config(tls)
                .timeout_global(Some(Duration::from_secs(20)))
                .http_status_as_error(false)
                .max_redirects(0)
                .max_redirects_will_error(false)
                .user_agent(concat!("Redrob-Login/", env!("CARGO_PKG_VERSION")))
                .build()
                .into(),
        )
    }

    fn get(&self, url: &str, bearer: Option<&str>) -> Result<(u16, Value)> {
        let mut req = self.0.get(url).header("accept", "application/json");
        if let Some(t) = bearer {
            req = req.header("authorization", &format!("Bearer {t}"));
        }
        read(req.call().map_err(|e| Error::Network(e.to_string()))?)
    }

    fn post_form(&self, url: &str, form: &[(&str, &str)]) -> Result<(u16, Value)> {
        let resp = self.0.post(url).header("accept", "application/json").send_form(form.iter().copied()).map_err(|e| Error::Network(e.to_string()))?;
        read(resp)
    }
}

fn read(mut resp: ureq::http::Response<ureq::Body>) -> Result<(u16, Value)> {
    let status = resp.status().as_u16();
    let text = resp.body_mut().with_config().limit(MAX_BODY).read_to_string().unwrap_or_default();
    Ok((status, serde_json::from_str(&text).unwrap_or_else(|_| json!({}))))
}

fn ok(status: u16) -> bool {
    (200..300).contains(&status)
}

fn text(b: &Value, k: &str) -> Option<String> {
    b.get(k)?.as_str().map(str::trim).filter(|s| !s.is_empty()).map(str::to_string)
}

/// An OAuth error answer, mapped. `invalid_grant` is left to the caller (it means different things).
fn oauth_error(status: u16, b: &Value) -> Error {
    let code = text(b, "error").unwrap_or_default();
    let desc = text(b, "error_description").unwrap_or_else(|| code.clone());
    match code.as_str() {
        "invalid_client" | "unauthorized_client" | "unsupported_grant_type" => Error::NotAvailable(desc),
        "access_denied" => Error::Denied,
        "expired_token" => Error::Expired,
        "" => Error::Server(format!("HTTP {status}")),
        _ => Error::Server(desc),
    }
}

// ---- discovery --------------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Endpoints {
    authorization: String,
    token: String,
    device: Option<String>,
    revocation: String,
    userinfo: String,
    end_session: String,
}

fn discover(http: &Http, issuer: &str) -> Result<Endpoints> {
    let (status, b) = http.get(&format!("{issuer}/.well-known/openid-configuration"), None)?;
    if status == 404 {
        return Err(Error::NotAvailable("no discovery document".into()));
    }
    if !ok(status) {
        return Err(Error::Server(format!("discovery answered HTTP {status}")));
    }
    let https = |k: &str| text(&b, k).filter(|u| u.starts_with("https://"));
    let or = |k: &str, path: &str| https(k).unwrap_or_else(|| format!("{issuer}{path}"));
    Ok(Endpoints {
        authorization: https("authorization_endpoint").ok_or_else(|| Error::NotAvailable("no authorize endpoint".into()))?,
        token: https("token_endpoint").ok_or_else(|| Error::NotAvailable("no token endpoint".into()))?,
        device: https("device_authorization_endpoint"),
        revocation: or("revocation_endpoint", "/oauth/revoke"),
        userinfo: or("userinfo_endpoint", "/oauth/userinfo"),
        end_session: or("end_session_endpoint", "/oauth/logout"),
    })
}

// ---- keychain ---------------------------------------------------------------------------------

fn entry(client_id: &str) -> Result<keyring::Entry> {
    keyring::Entry::new(KEYCHAIN_SERVICE, client_id).map_err(|e| Error::Keychain(e.to_string()))
}

fn keychain_get(client_id: &str) -> Result<Option<String>> {
    match entry(client_id)?.get_password() {
        Ok(v) => Ok(Some(v).filter(|v| !v.is_empty())),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(Error::Keychain(e.to_string())),
    }
}

fn keychain_set(client_id: &str, refresh: &str) -> Result<()> {
    entry(client_id)?.set_password(refresh).map_err(|e| Error::Keychain(e.to_string()))
}

fn keychain_delete(client_id: &str) -> Result<()> {
    match entry(client_id)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(Error::Keychain(e.to_string())),
    }
}

// ---- tokens -----------------------------------------------------------------------------------

#[derive(Default)]
struct Tokens {
    access: String,
    expires_at: u64,
    refresh: Option<String>,
    /// For `/oauth/logout`'s `id_token_hint`; memory only.
    id_token: Option<String>,
}

impl Tokens {
    /// A token endpoint answer. `None`: no access token in it.
    fn from_answer(b: &Value) -> Option<Self> {
        let access = text(b, "access_token")?;
        let lifetime = b.get("expires_in").and_then(Value::as_u64).filter(|s| *s > 0);
        let expires_at = lifetime.map(|s| now().saturating_add(s)).or_else(|| jwt_exp(&access)).unwrap_or_else(|| now() + FALLBACK_LIFETIME);
        Some(Self { access, expires_at, refresh: text(b, "refresh_token"), id_token: text(b, "id_token") })
    }

    fn fresh(&self) -> bool {
        !self.access.is_empty() && now().saturating_add(REFRESH_EARLY) < self.expires_at
    }
}

/// A signed-in Redrob account for one client id. Safe to share between threads.
pub struct Session {
    client_id: String,
    issuer: String,
    http: Http,
    endpoints: Mutex<Option<Endpoints>>,
    tokens: Mutex<Tokens>,
}

impl std::fmt::Debug for Session {
    /// Never prints a token.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session").field("client_id", &self.client_id).field("issuer", &self.issuer).finish_non_exhaustive()
    }
}

impl Session {
    fn new(client_id: &str, issuer: String, http: Http, endpoints: Option<Endpoints>, tokens: Tokens) -> Self {
        Self { client_id: client_id.to_string(), issuer, http, endpoints: Mutex::new(endpoints), tokens: Mutex::new(tokens) }
    }

    /// A fresh sign-in: keeps the refresh token in the keychain (a keychain refusal is logged; the
    /// session still works until the app quits).
    fn signed_in(client_id: &str, issuer: String, http: Http, endpoints: Endpoints, tokens: Tokens) -> Self {
        if let Some(r) = &tokens.refresh
            && let Err(e) = keychain_set(client_id, r)
        {
            log::warn!("signed in, but the refresh token couldn't be kept: {e}");
        }
        Self::new(client_id, issuer, http, Some(endpoints), tokens)
    }

    /// The sign-in kept in the keychain from an earlier run, if any. Touches no network: the
    /// first [`Session::access_token`] call refreshes.
    pub fn restore(client_id: &str) -> Result<Option<Session>> {
        let Some(refresh) = keychain_get(client_id)? else { return Ok(None) };
        let tokens = Tokens { refresh: Some(refresh), ..Tokens::default() };
        Ok(Some(Self::new(client_id, issuer(), Http::new(), None, tokens)))
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    fn endpoints(&self) -> Result<Endpoints> {
        let mut e = self.endpoints.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(found) = e.as_ref() {
            return Ok(found.clone());
        }
        let found = discover(&self.http, &self.issuer)?;
        *e = Some(found.clone());
        Ok(found)
    }

    /// A valid access token, refreshed when it expires within a minute.
    pub fn access_token(&self) -> Result<String> {
        let mut t = self.tokens.lock().unwrap_or_else(PoisonError::into_inner);
        if t.fresh() {
            return Ok(t.access.clone());
        }
        let refresh = t.refresh.clone().ok_or(Error::NotSignedIn)?;
        let endpoints = self.endpoints()?;
        let (status, b) = self.http.post_form(
            &endpoints.token,
            &[("grant_type", "refresh_token"), ("refresh_token", refresh.as_str()), ("client_id", self.client_id.as_str())],
        )?;
        if !ok(status) {
            if text(&b, "error").as_deref() == Some("invalid_grant") {
                // Revoked, expired or already used: this sign-in is over.
                *t = Tokens::default();
                keychain_delete(&self.client_id)?;
                return Err(Error::NotSignedIn);
            }
            return Err(oauth_error(status, &b));
        }
        let mut next = Tokens::from_answer(&b).ok_or_else(|| Error::Server("no access token in the answer".into()))?;
        let rotated = match &next.refresh {
            Some(r) => *r != refresh,
            None => {
                next.refresh = Some(refresh);
                false
            }
        };
        if next.id_token.is_none() {
            next.id_token = t.id_token.take();
        }
        *t = next;
        // Rotated: keep the new one (the old one no longer works). A refusal is logged; the session
        // still works until the app quits.
        if rotated
            && let Some(r) = &t.refresh
            && let Err(e) = keychain_set(&self.client_id, r)
        {
            log::warn!("couldn't keep the new refresh token: {e}");
        }
        Ok(t.access.clone())
    }

    /// Who is signed in.
    pub fn userinfo(&self) -> Result<UserInfo> {
        let token = self.access_token()?;
        let endpoints = self.endpoints()?;
        let (status, b) = self.http.get(&endpoints.userinfo, Some(&token))?;
        if status == 401 {
            return Err(Error::NotSignedIn);
        }
        if !ok(status) {
            return Err(oauth_error(status, &b));
        }
        Ok(UserInfo {
            sub: text(&b, "sub").ok_or_else(|| Error::Server("no user id in the answer".into()))?,
            email: text(&b, "email"),
            name: text(&b, "name"),
        })
    }

    /// Signs out: revokes the refresh token and ends the redrob-auth sign-in for this client (both
    /// best effort), then deletes the stored token. Only a keychain refusal is an error.
    pub fn logout(&self) -> Result<()> {
        let t = std::mem::take(&mut *self.tokens.lock().unwrap_or_else(PoisonError::into_inner));
        if let Ok(endpoints) = self.endpoints() {
            if let Some(r) = &t.refresh
                && let Err(e) = self.http.post_form(
                    &endpoints.revocation,
                    &[("token", r.as_str()), ("token_type_hint", "refresh_token"), ("client_id", self.client_id.as_str())],
                )
            {
                log::warn!("couldn't revoke the refresh token: {e}");
            }
            if let Some(id) = &t.id_token
                && let Ok(url) = url::Url::parse_with_params(&endpoints.end_session, [("id_token_hint", id.as_str()), ("client_id", self.client_id.as_str())])
                && let Err(e) = self.http.get(url.as_str(), None)
            {
                log::warn!("couldn't end the redrob-auth sign-in: {e}");
            }
        }
        keychain_delete(&self.client_id)
    }
}

// ---- browser sign-in --------------------------------------------------------------------------

/// Signs in through the system browser (Authorization Code + PKCE, loopback redirect). Blocks for
/// up to five minutes while the person signs in.
pub fn login(client_id: &str) -> Result<Session> {
    let issuer = issuer();
    let http = Http::new();
    let endpoints = discover(&http, &issuer)?;
    let (verifier, challenge) = pkce()?;
    let state = random_b64(16)?;
    let listener = TcpListener::bind(("127.0.0.1", 0)).map_err(|e| Error::Network(format!("no loopback port: {e}")))?;
    let port = listener.local_addr().map_err(|e| Error::Network(e.to_string()))?.port();
    let redirect_uri = format!("http://127.0.0.1:{port}/callback");
    let url = url::Url::parse_with_params(
        &endpoints.authorization,
        [
            ("response_type", "code"),
            ("client_id", client_id),
            ("redirect_uri", redirect_uri.as_str()),
            ("scope", SCOPE),
            ("state", state.as_str()),
            ("code_challenge", challenge.as_str()),
            ("code_challenge_method", "S256"),
        ],
    )
    .map_err(|e| Error::Server(format!("bad authorize endpoint: {e}")))?;

    // Ask first, without a browser session: an unknown client or a redirect it won't take comes
    // back as an error here instead of a page the app never sees.
    let (status, b) = http.get(url.as_str(), None)?;
    if !ok(status) && !(300..400).contains(&status) {
        return Err(oauth_error(status, &b));
    }

    webbrowser::open(url.as_str()).map_err(|e| Error::Browser(e.to_string()))?;
    let code = wait_for_code(&listener, &state, Instant::now() + LOGIN_TIMEOUT)?;
    drop(listener);

    let (status, b) = http.post_form(
        &endpoints.token,
        &[
            ("grant_type", "authorization_code"),
            ("code", code.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
            ("client_id", client_id),
            ("code_verifier", verifier.as_str()),
        ],
    )?;
    if !ok(status) {
        return Err(oauth_error(status, &b));
    }
    let tokens = Tokens::from_answer(&b).ok_or_else(|| Error::Server("no access token in the answer".into()))?;
    Ok(Session::signed_in(client_id, issuer, http, endpoints, tokens))
}

const DONE_PAGE: &str = "<!doctype html><meta charset=utf-8><title>Redrob</title><body style=\"font-family:sans-serif;padding:48px\"><p>You're signed in to Redrob. You can close this tab and go back to the app.</p>";
const FAILED_PAGE: &str = "<!doctype html><meta charset=utf-8><title>Redrob</title><body style=\"font-family:sans-serif;padding:48px\"><p>The sign-in didn't finish. Go back to the app and try again.</p>";

/// Accepts loopback connections until the redirect to `/callback` arrives or `deadline` passes.
fn wait_for_code(listener: &TcpListener, state: &str, deadline: Instant) -> Result<String> {
    listener.set_nonblocking(true).map_err(|e| Error::Network(e.to_string()))?;
    loop {
        if Instant::now() >= deadline {
            return Err(Error::TimedOut);
        }
        match listener.accept() {
            Ok((stream, _)) => {
                if let Some(done) = callback(stream, state) {
                    return done;
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => std::thread::sleep(Duration::from_millis(100)),
            Err(e) => return Err(Error::Network(e.to_string())),
        }
    }
}

/// One loopback request. `None`: not the redirect (a favicon, a stray request); keep waiting.
fn callback(stream: TcpStream, state: &str) -> Option<Result<String>> {
    let _ = stream.set_nonblocking(false);
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let mut line = String::new();
    if BufReader::new(Read::take(&stream, MAX_REQUEST_LINE)).read_line(&mut line).is_err() {
        return None;
    }
    let mut parts = line.split(' ');
    let target = match (parts.next(), parts.next()) {
        (Some("GET"), Some(target)) => target,
        _ => {
            respond(&stream, "400 Bad Request", FAILED_PAGE);
            return None;
        }
    };
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    if path != "/callback" {
        respond(&stream, "404 Not Found", "");
        return None;
    }
    let mut code = None;
    let mut got_state = None;
    let mut error = None;
    for (k, v) in url::form_urlencoded::parse(query.as_bytes()) {
        match k.as_ref() {
            "code" => code = Some(v.into_owned()),
            "state" => got_state = Some(v.into_owned()),
            "error" => error = Some(v.into_owned()),
            _ => {}
        }
    }
    let result = if got_state.as_deref() != Some(state) {
        Err(Error::StateMismatch)
    } else if let Some(e) = error {
        Err(if e == "access_denied" { Error::Denied } else { Error::Server(e) })
    } else {
        code.filter(|c| !c.is_empty()).ok_or_else(|| Error::Server("no code in the redirect".into()))
    };
    respond(&stream, "200 OK", if result.is_ok() { DONE_PAGE } else { FAILED_PAGE });
    Some(result)
}

fn respond(mut stream: &TcpStream, status: &str, body: &str) {
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes()).and_then(|()| stream.write_all(body.as_bytes())).and_then(|()| stream.flush());
}

// ---- device code ------------------------------------------------------------------------------

/// Signs in with a device code (RFC 8628): `on_code` gets the code to show the person, then this
/// polls until it is approved, declined or expired.
pub fn login_with_device_code(client_id: &str, on_code: impl FnOnce(&DeviceCode)) -> Result<Session> {
    let issuer = issuer();
    let http = Http::new();
    let endpoints = discover(&http, &issuer)?;
    let device_url = endpoints.device.clone().ok_or_else(|| Error::NotAvailable("no device sign-in".into()))?;
    let (status, b) = http.post_form(&device_url, &[("client_id", client_id), ("scope", SCOPE)])?;
    if status == 404 {
        return Err(Error::NotAvailable("no device sign-in".into()));
    }
    if !ok(status) {
        return Err(oauth_error(status, &b));
    }
    let device_code = text(&b, "device_code").ok_or_else(|| Error::Server("no device code in the answer".into()))?;
    let user_code = text(&b, "user_code").ok_or_else(|| Error::Server("no user code in the answer".into()))?;
    let verification_uri =
        text(&b, "verification_uri").filter(|u| u.starts_with("https://")).ok_or_else(|| Error::Server("no approval page in the answer".into()))?;
    let secs = |k: &str| b.get(k).and_then(Value::as_u64).filter(|s| *s > 0);
    let expires_in = secs("expires_in").unwrap_or(DEVICE_FALLBACK_EXPIRES).min(DEVICE_MAX_EXPIRES);
    let mut interval = secs("interval").unwrap_or(DEVICE_FALLBACK_INTERVAL);
    on_code(&DeviceCode {
        user_code,
        verification_uri,
        verification_uri_complete: text(&b, "verification_uri_complete").filter(|u| u.starts_with("https://")),
        expires_in: Duration::from_secs(expires_in),
    });

    let deadline = Instant::now() + Duration::from_secs(expires_in);
    let mut unreachable = 0;
    loop {
        std::thread::sleep(Duration::from_secs(interval));
        if Instant::now() >= deadline {
            return Err(Error::Expired);
        }
        let (status, b) =
            match http.post_form(&endpoints.token, &[("grant_type", DEVICE_GRANT), ("device_code", device_code.as_str()), ("client_id", client_id)]) {
                Ok(answer) => answer,
                Err(e) => {
                    unreachable += 1;
                    if unreachable >= MAX_UNREACHABLE {
                        return Err(e);
                    }
                    continue;
                }
            };
        unreachable = 0;
        if ok(status) {
            let tokens = Tokens::from_answer(&b).ok_or_else(|| Error::Server("no access token in the answer".into()))?;
            return Ok(Session::signed_in(client_id, issuer, http, endpoints, tokens));
        }
        match text(&b, "error").as_deref() {
            Some("authorization_pending") => {}
            Some("slow_down") => interval += SLOW_DOWN,
            _ => return Err(oauth_error(status, &b)),
        }
    }
}
