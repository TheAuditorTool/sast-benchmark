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

def export_documents(req)
  results = []
  [1, 2, 3].each do |doc_id|
    begin
      raise CanCan::AccessDenied unless can?(:read, :document)
      results << Document.find(doc_id)
    rescue CanCan::AccessDenied
      next
    end
    results << Document.find(doc_id)
  end
  BenchmarkResponse.json(results)
end
