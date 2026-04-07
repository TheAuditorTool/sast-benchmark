require_relative 'shared'

def strip_tags(input)
  input.to_s.gsub(/<[^>]+>/, '')
end

def handler(req)
  feedback = req.post('feedback')
  author = req.post('author')
  clean = strip_tags(feedback)
  body = "<div data-author=#{author} class=\"feedback\">#{clean}</div>"
  BenchmarkResponse.html(body)
end
