require_relative 'shared'

def post_comment_origin_check(req)
  origin = req.header('Origin')
  allowed_origin = 'https://app.example.com'
  unless origin == allowed_origin
    return BenchmarkResponse.bad_request('origin not allowed')
  end
  comment = req.post('comment')
  BenchmarkResponse.json({ result: comment })
end
