class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "test3", "description" => "hello, I'm test3" }
        return grades
    end

    def passive_run(index)
        puts self.metadata
        puts "Scan payload test3"
    end
end