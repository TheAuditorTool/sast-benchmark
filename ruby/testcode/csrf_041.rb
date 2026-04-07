require_relative 'shared'

# vuln-code-snippet start ruby_csrf_origin_strict
def post_comment_origin_check(req)
  origin = req.header('Origin')
  allowed_origin = 'https://app.example.com'
  unless origin == allowed_origin  # vuln-code-snippet safe-line ruby_csrf_origin_strict
    return BenchmarkResponse.bad_request('origin not allowed')
  end
  comment = req.post('comment')
  BenchmarkResponse.json({ result: comment })
end
# vuln-code-snippet end ruby_csrf_origin_strict
