require_relative 'shared'

module FluentLogger
  def self.post(tag, data)
    $stdout.puts("fluent:#{tag} #{data.inspect}")
  end
end

def log_fluent_tag(req)
  FluentLogger.post(req.param('tag'), { event: 'login', time: Time.now.to_i })
  BenchmarkResponse.json({ ok: true })
end
