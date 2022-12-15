class RBModule_test
    def initialize
    end
    def metadata
        info = { "name" => "test", 
            "description" => "" }
        return info
    end

    def passive_run(index)
        puts Similary.match("abcdefg","bbcdefg")
    end

end