require_relative 'shared'

ALLOWED_THEMES = %w[light dark high-contrast].freeze

def set_theme_header(req)
  requested = req.param('theme')
  theme = ALLOWED_THEMES.include?(requested) ? requested : 'light'
  headers = { 'X-Theme' => theme }
  BenchmarkResponse.new(200, 'ok', headers)
end
