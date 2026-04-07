require_relative 'shared'

begin; require 'syslog'; rescue LoadError; end

module Syslog
  LOG_INFO = 6
  def self.open(ident, logopt = 0, facility = 0); yield self if block_given?; end
  def self.log(priority, *args); end
end unless defined?(Syslog::LOG_INFO)

def log_syslog(req)
  username = req.param('username')
  Syslog.open('benchmark') do |s|
    s.log(Syslog::LOG_INFO, username)
  end
  BenchmarkResponse.ok('logged')
end
