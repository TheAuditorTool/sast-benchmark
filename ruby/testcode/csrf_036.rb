require_relative 'shared'

# vuln-code-snippet start ruby_csrf_custom_header_check
def ajax_action_safe(req)
  xhr = req.header('X-Requested-With')
  origin = req.header('Origin')
  # Custom header not sent by cross-origin browser forms — safe for same-origin AJAX
  unless xhr == 'XMLHttpRequest' && (origin.nil? || origin == 'https://app.example.com')  # vuln-code-snippet safe-line ruby_csrf_custom_header_check
    return BenchmarkResponse.bad_request('not an XHR from allowed origin')
  end
  BenchmarkResponse.json({ result: true })
end
# vuln-code-snippet end ruby_csrf_custom_header_check
