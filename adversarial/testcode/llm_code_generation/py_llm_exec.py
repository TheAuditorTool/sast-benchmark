"""
LLM Code Generation Test Cases (Python)
Tests SAST tool ability to detect LLM-driven dynamic code assembly.

Background: Unit 42 (January 2026) demonstrated real-world LLM runtime
assembly where webpages call LLM APIs to generate unique code per victim.
nullifAI (February 2025) used malicious PyTorch models on Hugging Face
to execute payloads during model loading.

Reference: https://unit42.paloaltonetworks.com/real-time-malicious-javascript-through-llms/
"""
import json
import urllib.request


# ============================================================================
# VULNERABLE: Anthropic API response fed directly to exec()
# ============================================================================

# vuln-code-snippet start anthropic_response_exec
def generate_and_run_migration(db_schema):
    """Generate a database migration using Claude and execute it.

    The LLM response is treated as trusted code and executed directly.
    An attacker who controls the prompt (via injection in db_schema)
    or who performs a MITM on the API call can inject arbitrary Python.
    """
    import anthropic

    client = anthropic.Anthropic()
    message = client.messages.create(
        model="claude-sonnet-4-20250514",
        max_tokens=1024,
        messages=[{
            "role": "user",
            "content": f"Generate Python migration code for this schema:\n{db_schema}",
        }],
    )
    migration_code = message.content[0].text
    exec(migration_code)  # vuln-code-snippet vuln-line anthropic_response_exec
# vuln-code-snippet end anthropic_response_exec


# ============================================================================
# VULNERABLE: Hugging Face model output executed as code
# Pattern from nullifAI campaign (Feb 2025)
# ============================================================================

# vuln-code-snippet start huggingface_model_exec
def run_model_generated_code(model_name, prompt):
    """Load a code generation model and execute its output.

    The nullifAI campaign (Feb 2025) demonstrated that malicious models
    on Hugging Face can execute arbitrary code during loading via
    crafted Pickle payloads. Even if loading is safe, executing the
    model's text output as code is dangerous.
    """
    from transformers import pipeline

    generator = pipeline("text-generation", model=model_name)
    result = generator(prompt, max_length=512)
    generated_code = result[0]["generated_text"]
    exec(generated_code)  # vuln-code-snippet vuln-line huggingface_model_exec
# vuln-code-snippet end huggingface_model_exec


# ============================================================================
# SAFE: OpenAI API used for text classification (no code execution)
# ============================================================================

# vuln-code-snippet start openai_classification
def classify_support_ticket(ticket_text):
    """Classify a support ticket using OpenAI API.

    The API response is used as a classification label (string data),
    never passed to exec() or eval(). The output is constrained to
    a known set of categories.
    """
    data = json.dumps({
        "model": "gpt-4o",
        "messages": [
            {"role": "system", "content": "Classify this support ticket as one of: billing, technical, account, other. Respond with only the category name."},
            {"role": "user", "content": ticket_text},
        ],
        "max_tokens": 10,
    }).encode("utf-8")

    req = urllib.request.Request(
        "https://api.openai.com/v1/chat/completions",
        data=data,
        headers={"Content-Type": "application/json", "Authorization": "Bearer " + get_api_key()},
    )
    response = json.loads(urllib.request.urlopen(req).read())
    category = response["choices"][0]["message"]["content"].strip().lower()
    valid_categories = {"billing", "technical", "account", "other"}
    return category if category in valid_categories else "other"  # vuln-code-snippet safe-line openai_classification
# vuln-code-snippet end openai_classification


# ============================================================================
# SAFE: Hugging Face inference API for sentiment analysis (returns score)
# ============================================================================

# vuln-code-snippet start huggingface_inference
def analyze_sentiment(text):
    """Analyze text sentiment using Hugging Face Inference API.

    The API returns a numeric confidence score and a label string.
    Neither is executed as code. The output is used as structured data
    for display or storage.
    """
    data = json.dumps({"inputs": text}).encode("utf-8")

    req = urllib.request.Request(
        "https://api-inference.huggingface.co/models/distilbert-base-uncased-finetuned-sst-2-english",
        data=data,
        headers={"Authorization": "Bearer " + get_hf_token()},
    )
    response = json.loads(urllib.request.urlopen(req).read())
    top_result = max(response[0], key=lambda x: x["score"])
    return {"label": top_result["label"], "confidence": top_result["score"]}  # vuln-code-snippet safe-line huggingface_inference
# vuln-code-snippet end huggingface_inference


# ============================================================================
# VULNERABLE: PyTorch model loaded from untrusted source (pickle-based RCE)
# nullifAI pattern (Feb 2025) -- malicious pickle payload in model weights
# ============================================================================

# vuln-code-snippet start torch_load_untrusted
def load_community_model(model_path):
    """Load a community-contributed PyTorch model from a file.

    PyTorch's default serialization uses Python pickle, which allows
    arbitrary code execution during deserialization. The nullifAI campaign
    (Feb 2025) demonstrated this by uploading malicious models to Hugging
    Face Hub with crafted __reduce__ methods that execute reverse shells
    during torch.load(). The models looked functional but contained hidden
    pickle payloads that ran before any inference code.
    """
    import torch

    # torch.load() uses pickle.load() internally -- arbitrary code execution
    model = torch.load(model_path)  # vuln-code-snippet vuln-line torch_load_untrusted
    return model
# vuln-code-snippet end torch_load_untrusted


# ============================================================================
# SAFE: Model loaded using safetensors format (no code execution by design)
# ============================================================================

# vuln-code-snippet start torch_load_safetensors
def load_verified_model(model_path):
    """Load a model using safetensors format.

    safetensors (by Hugging Face) is a serialization format that stores
    only tensor data -- no arbitrary Python objects, no pickle, no code
    execution. It was created specifically to prevent the class of attacks
    demonstrated by nullifAI. Loading a safetensors file cannot execute code.
    """
    from safetensors.torch import load_file

    # safetensors only deserializes numeric tensor data, never code
    tensors = load_file(model_path)  # vuln-code-snippet safe-line torch_load_safetensors
    return tensors
# vuln-code-snippet end torch_load_safetensors


def get_api_key():
    """Retrieve OpenAI API key from environment."""
    import os
    return os.environ.get("OPENAI_API_KEY", "")


def get_hf_token():
    """Retrieve Hugging Face token from environment."""
    import os
    return os.environ.get("HF_TOKEN", "")
