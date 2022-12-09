class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "test2", "description" => "hello, I'm test1" }
        return grades
    end

    def passive_run(index)
        puts self.metadata
        puts "Scan payload test2"
    end
end