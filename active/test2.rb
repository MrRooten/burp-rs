require_relative "logger/logger"
require_relative "http/http"
class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "asdf", 
            "description" => "" }
        return grades
    end

    def passive_run(index)
        logger = Logger.new
        logger.info("hello")
    end

end


