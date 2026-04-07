require_relative 'shared'

require 'openssl'

def delete_item_per_action_token(req)
  item_id = req.param('id').to_i
  provided_token = req.post('action_token')
  session_key = req.cookie('session_key').to_s
  expected_token = OpenSSL::HMAC.hexdigest('SHA256', session_key, "delete_item_#{item_id}")
  unless Rack::Utils.secure_compare(provided_token.to_s, expected_token)
    return BenchmarkResponse.bad_request('invalid action token')
  end
  BenchmarkResponse.json({ result: item_id })
end
