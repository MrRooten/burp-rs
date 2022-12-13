class Logger
    def initialize
        @logger = RBLogger.new
    end

    def info(s)
        @logger.info("#{__FILE__} #{__LINE__}: "+s)
    end

    def error(s)
        @logger.error("#{__FILE__} #{__LINE__}: "+s)
    end

    def warn(s)
        @logger.warn("#{__FILE__} #{__LINE__}: "+s)
    end

    def debug(s)
        @logger.debug("#{__FILE__} #{__LINE__}: "+s)
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