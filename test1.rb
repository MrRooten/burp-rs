class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "test1", "description" => "hello, I'm test1" }
        return grades
    end

    def passive_run
        print("hello, I'm test1")
    end
end