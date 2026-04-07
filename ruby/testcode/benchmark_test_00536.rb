require_relative 'shared'

def set_computed_content_length(req)
  body = JSON.generate({ ok: true })
  response = BenchmarkResponse.ok(body)
  response.headers['Content-Length'] = body.bytesize.to_s
  response
end
