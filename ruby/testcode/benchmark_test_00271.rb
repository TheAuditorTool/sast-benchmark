require_relative 'shared'

def safe_redirect(req)
  url = req.param('url')
  if url.start_with?('/') && !url.start_with?('//')
    BenchmarkResponse.redirect(url)
  else
    BenchmarkResponse.bad_request('invalid redirect target')
  end
end
