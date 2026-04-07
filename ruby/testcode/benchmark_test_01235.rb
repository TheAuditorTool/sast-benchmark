require_relative 'shared'

begin; require 'patron'; rescue LoadError; end

def fetch_patron(req)
  sess = Patron::Session.new
  sess.get(req.param('url'))
  BenchmarkResponse.json({ ok: true })
end
