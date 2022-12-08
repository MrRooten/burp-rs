class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "test3", "description" => "hello, I'm test3" }
        return grades
    end

    def passive_run
        puts self.method('passive_run').object_id
        puts "test3: " + self.method('passive_run').object_id.to_s
    end
end