require_relative 'shared'

module FluentLogger
  def self.post(tag, data)
    # stub: would send to fluentd
    $stdout.puts("fluent:#{tag} #{data.inspect}")
  end
end

# vuln-code-snippet start ruby_loginj_fluent_tag
def log_fluent_tag(req)
  FluentLogger.post(req.param('tag'), { event: 'login', time: Time.now.to_i }) # vuln-code-snippet vuln-line ruby_loginj_fluent_tag
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_fluent_tag
