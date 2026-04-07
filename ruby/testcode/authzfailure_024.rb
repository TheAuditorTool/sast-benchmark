require_relative 'shared'

module CanCan
  class AccessDenied < StandardError; end
end

class Document
  def self.find(id)
    { id: id, content: "sensitive document #{id}" }
  end
end

def can?(action, resource)
  false
end

# vuln-code-snippet start ruby_authz_cancan_rescued
def export_documents(req)
  results = []
  [1, 2, 3].each do |doc_id|
    begin
      raise CanCan::AccessDenied unless can?(:read, :document)
      results << Document.find(doc_id)
    rescue CanCan::AccessDenied # vuln-code-snippet vuln-line ruby_authz_cancan_rescued
      next
    end
    results << Document.find(doc_id)
  end
  BenchmarkResponse.json(results)
end
# vuln-code-snippet end ruby_authz_cancan_rescued
