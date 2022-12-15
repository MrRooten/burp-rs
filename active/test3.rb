class RBModule_test3
    def initialize
    end
    def metadata
        grades = { "name" => "test3", 
            "description" => "" }
        return grades
    end

    def passive_run(index)
        puts Similary.match("abcdefg","bbcdefg")
    end

end