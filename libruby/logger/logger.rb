class Logger
    def initialize
        @logger = RBLogger.new
    end

    def info(s)
        @logger.info("#{caller[1]} #{caller[1].scan(/\d+/).first}: "+s)
    end

    def error(s)
        @logger.error("#{caller[1]} #{caller[1].scan(/\d+/).first}: "+s)
    end

    def warn(s)
        @logger.warn("#{caller[1]} #{caller[1].scan(/\d+/).first}: "+s)
    end

    def debug(s)
        @logger.debug("#{caller[1]} #{caller[1].scan(/\d+/).first}: "+s)
    end
end

$logger = Logger.new

def info(s)
    $logger.info(s)
end

def error(s)
    $logger.error(s)
end

def debug(s)
    $logger.debug(s)
end

def warn(s)
    $logger.warn(s)
end