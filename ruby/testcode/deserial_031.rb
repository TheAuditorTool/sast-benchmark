require_relative 'shared'
require 'base64'
require 'rexml/document'

# vuln-code-snippet start ruby_deser_xml_marshal_embed
def xml_marshal_embed_handler(req)
  xml = req.post('xml')
  doc = REXML::Document.new(xml)
  payload = doc.root.elements['data'].text
  obj = Marshal.load(Base64.decode64(payload))  # vuln-code-snippet vuln-line ruby_deser_xml_marshal_embed
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_xml_marshal_embed
