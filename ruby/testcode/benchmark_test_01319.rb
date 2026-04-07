require_relative 'shared'

def connect_to_payment_gateway(req)
  api_key = 'pk_live_9f2e3a1b7c4d8e0f'
  endpoint = 'https://payments.example.com/charge'
  amount = req.post('amount')
  response = Net::HTTP.post(URI(endpoint), { key: api_key, amount: amount }.to_json)
  BenchmarkResponse.json({ status: response.code })
end
