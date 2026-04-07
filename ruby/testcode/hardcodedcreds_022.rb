require_relative 'shared'

TWILIO_TOKEN = "ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
TWILIO_SID   = "SKxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"

# vuln-code-snippet start ruby_hardcoded_twilio
def send_sms_handler(req)
  client = Twilio::REST::Client.new(TWILIO_SID, TWILIO_TOKEN)  # vuln-code-snippet vuln-line ruby_hardcoded_twilio
  client.messages.create(
    from: '+15551234567',
    to:   req[:to],
    body: req[:message]
  )
  BenchmarkResponse.ok('sms sent')
end
# vuln-code-snippet end ruby_hardcoded_twilio
