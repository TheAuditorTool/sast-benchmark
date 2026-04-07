require_relative 'shared'

class UserProjectCollection
  def initialize(user_id)
    @user_id = user_id
  end

  def find(id)
    raise 'not found' unless "user_#{id.to_i % 5}" == @user_id
    { id: id, name: "project_#{id}", owner_id: @user_id }
  end
end

class TaskRecord
  def self.where(project_id:)
    [{ id: 1, title: 'Task alpha', project_id: project_id },
     { id: 2, title: 'Task beta',  project_id: project_id }]
  end
end

def current_user_projects(user_id)
  UserProjectCollection.new(user_id)
end

def list_project_tasks(req)
  project_id = req.param('project_id')
  current_user_id = req.cookie('user_id')
  project = current_user_projects(current_user_id).find(project_id)
  tasks = TaskRecord.where(project_id: project[:id])
  BenchmarkResponse.json(tasks)
end
