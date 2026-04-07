require_relative 'shared'

def load_preferences(req)
  raw = req.body_str
  prefs = YAML.safe_load(raw)
  BenchmarkResponse.json({ theme: prefs['theme'], lang: prefs['lang'] })
end
