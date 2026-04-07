require_relative 'shared'

def store_password_xor(req)
  password = req.param('password')
  key      = req.param('key')
  digest = password.bytes.zip(key.bytes.cycle).map { |a, b| a ^ b }.pack('C*')
  BenchmarkResponse.json({ hash: digest.unpack1('H*') })
end
