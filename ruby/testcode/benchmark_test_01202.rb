require_relative 'shared'

TWILIO_TOKEN = "ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
TWILIO_SID   = "SKxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"

def send_sms_handler(req)
  client = Twilio::REST::Client.new(TWILIO_SID, TWILIO_TOKEN)
  client.messages.create(
    from: '+15551234567',
    to:   req[:to],
    body: req[:message]
  )
  BenchmarkResponse.ok('sms sent')
end
