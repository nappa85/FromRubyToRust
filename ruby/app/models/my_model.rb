class MyModel < ActiveRecord::Base
    self.table_name = 'my_table'

    scope :active, -> { where(status: 'Active') }

    serialize :metadata, JSON

    def cached_method
        @cache ||= self.class.active.do_something_expensive(id: id)
    end

    def self.do_something_expensive(id:)
        select do |model|
            model.metadata['parent_id'] == id
        end
    end
end
