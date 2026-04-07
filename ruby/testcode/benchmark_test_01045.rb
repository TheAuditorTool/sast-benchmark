require_relative 'shared'

def sanitize_strict(input)
  input.to_s.gsub(/<[^>]*>/, '')
            .gsub(/javascript:/i, '')
            .gsub(/on\w+\s*=/i, '')
end

def handler(req)
  bio = req.post('bio')
  display_name = req.post('display_name')
  clean_bio = sanitize_strict(bio)
  clean_name = sanitize_strict(display_name)
  BenchmarkResponse.html("<section><h2>#{clean_name}</h2><p>#{clean_bio}</p></section>")
end
