// Interactor

struct FinancialEntity {}

trait FinancialDataGateway {
    fn get_entities(&self) -> Vec<FinancialEntity>;
}

impl FinancialDataGateway for FinancialDataMapper {
    fn get_entities(&self) -> Vec<FinancialEntity> {
        vec![FinancialEntity {}]
    }
}

trait FinancialReportRequester {
    fn get_entities(&self) -> Vec<FinancialEntity>;
}

struct FinancialReportGenerator<'a> {
    database_gateway: Box<dyn FinancialDataGateway + 'a>,
}

impl<'a> FinancialReportGenerator<'a> {
    fn new(database_gateway: Box<dyn FinancialDataGateway + 'a>) -> Self {
        FinancialReportGenerator { database_gateway }
    }
}

impl<'a> FinancialReportRequester for FinancialReportGenerator<'a> {
    fn get_entities(&self) -> Vec<FinancialEntity> {
        self.database_gateway.get_entities()
    }
}

// Database

struct FinancialDatabase {}

struct FinancialDataMapper {
    db: FinancialDatabase,
}

impl FinancialDataMapper {
    fn new(db: FinancialDatabase) -> Self {
        FinancialDataMapper { db }
    }
}
