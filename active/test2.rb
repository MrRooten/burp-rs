class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "test2", "description" => "hello, I'm test1" }
        return grades
    end

    def passive_run
        puts self.method('passive_run').object_id
        puts "test2: " + self.method('passive_run').object_id.to_s
    end
end