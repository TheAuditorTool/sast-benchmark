require_relative 'shared'

class Project
  attr_reader :id, :user_id, :name

  def initialize(id, user_id, name)
    @id = id
    @user_id = user_id
    @name = name
  end

  def self.find(id)
    new(id, "user_#{id.to_i % 5}", "project_#{id}")
  end
end

def delete_project(req)
  project_id = req.param('id')
  current_user_id = req.cookie('user_id')
  record = Project.find(project_id)
  raise 'forbidden' unless record.user_id == current_user_id
  BenchmarkResponse.ok("project #{project_id} deleted")
end
