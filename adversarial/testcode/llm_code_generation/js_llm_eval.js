/**
 * LLM Code Generation Test Cases (JavaScript)
 * Tests SAST tool ability to detect LLM-driven dynamic code assembly.
 *
 * Based on: Unit 42 (Jan 2026) demonstrated real-world LLM runtime code
 * assembly where webpages embed prompts that call LLM APIs client-side
 * to generate unique, polymorphic JavaScript per victim. Traffic travels
 * over trusted LLM API domains, evading network analysis.
 *
 * Reference: https://unit42.paloaltonetworks.com/real-time-malicious-javascript-through-llms/
 */


// ============================================================================
// VULNERABLE: OpenAI API response fed to eval()
// Unit 42 "LLM runtime assembly" pattern
// ============================================================================

// vuln-code-snippet start openai_response_eval
async function generateDynamicWidget(userPreferences) {
    // Call OpenAI API to generate a UI widget based on user preferences.
    // The generated code is eval'd to create a "personalized experience."
    // An attacker who controls userPreferences (or performs prompt injection)
    // can cause the LLM to generate malicious JavaScript.
    const response = await fetch('https://api.openai.com/v1/chat/completions', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${process.env.OPENAI_API_KEY}`,
        },
        body: JSON.stringify({
            model: 'gpt-4o',
            messages: [{
                role: 'user',
                content: `Generate JavaScript code for a widget with these preferences: ${JSON.stringify(userPreferences)}`,
            }],
        }),
    });
    const data = await response.json();
    const widgetCode = data.choices[0].message.content;
    eval(widgetCode);  // vuln-code-snippet vuln-line openai_response_eval
}
// vuln-code-snippet end openai_response_eval


// ============================================================================
// VULNERABLE: LLM generates HTML template injected via innerHTML
// ============================================================================

// vuln-code-snippet start llm_template_inject
async function renderAIGeneratedPage(topic) {
    // Ask an LLM to generate an HTML page about a topic.
    // The response is inserted directly into the DOM via innerHTML.
    // If the LLM output contains <script> tags or event handlers,
    // they execute in the user's browser context.
    const response = await fetch('https://api.openai.com/v1/chat/completions', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${process.env.OPENAI_API_KEY}`,
        },
        body: JSON.stringify({
            model: 'gpt-4o',
            messages: [{
                role: 'user',
                content: `Generate an HTML page about: ${topic}`,
            }],
        }),
    });
    const data = await response.json();
    const html = data.choices[0].message.content;
    document.getElementById('content').innerHTML = html;  // vuln-code-snippet vuln-line llm_template_inject
}
// vuln-code-snippet end llm_template_inject


// ============================================================================
// SAFE: Claude API for text summarization (display as text, no execution)
// ============================================================================

// vuln-code-snippet start anthropic_summarize
async function summarizeArticle(articleText) {
    // Use Claude to summarize an article. The summary is displayed
    // as plain text content, never executed or inserted as HTML.
    const response = await fetch('https://api.anthropic.com/v1/messages', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'x-api-key': process.env.ANTHROPIC_API_KEY,
            'anthropic-version': '2023-06-01',
        },
        body: JSON.stringify({
            model: 'claude-sonnet-4-20250514',
            max_tokens: 256,
            messages: [{
                role: 'user',
                content: `Summarize this article in 2-3 sentences:\n\n${articleText}`,
            }],
        }),
    });
    const data = await response.json();
    const summary = data.content[0].text;
    document.getElementById('summary').textContent = summary;  // vuln-code-snippet safe-line anthropic_summarize
}
// vuln-code-snippet end anthropic_summarize


// ============================================================================
// SAFE: LLM reviews code and returns structured JSON comments (no execution)
// ============================================================================

// vuln-code-snippet start llm_code_review_display
async function getCodeReview(sourceCode) {
    // Use an LLM to review code and return structured feedback.
    // The response is parsed as JSON and displayed in a review UI.
    // The review comments are never executed, only rendered as text.
    const response = await fetch('https://api.openai.com/v1/chat/completions', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${process.env.OPENAI_API_KEY}`,
        },
        body: JSON.stringify({
            model: 'gpt-4o',
            response_format: { type: 'json_object' },
            messages: [{
                role: 'user',
                content: `Review this code. Respond in JSON: {"comments": [{"line": N, "severity": "info|warn|error", "message": "..."}]}\n\n${sourceCode}`,
            }],
        }),
    });
    const data = await response.json();
    const review = JSON.parse(data.choices[0].message.content);
    return review.comments;  // vuln-code-snippet safe-line llm_code_review_display
}
// vuln-code-snippet end llm_code_review_display
