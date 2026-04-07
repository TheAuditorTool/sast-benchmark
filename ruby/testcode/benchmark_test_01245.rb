require_relative 'shared'
require 'base64'
require 'rexml/document'

def xml_marshal_embed_handler(req)
  xml = req.post('xml')
  doc = REXML::Document.new(xml)
  payload = doc.root.elements['data'].text
  obj = Marshal.load(Base64.decode64(payload))
  BenchmarkResponse.json({ result: obj.to_s })
end
