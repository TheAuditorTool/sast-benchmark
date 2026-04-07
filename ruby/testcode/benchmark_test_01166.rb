require_relative 'shared'

def ajax_action_safe(req)
  xhr = req.header('X-Requested-With')
  origin = req.header('Origin')
  unless xhr == 'XMLHttpRequest' && (origin.nil? || origin == 'https://app.example.com')
    return BenchmarkResponse.bad_request('not an XHR from allowed origin')
  end
  BenchmarkResponse.json({ result: true })
end
