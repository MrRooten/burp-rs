class Logger
    def initialize
        @logger = RBLogger.new
    end

    def info(s)
        @logger.info("#{__FILE__} #{__LINE__}: "+s)
    end
end
