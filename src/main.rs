
use rusqlite;
//This is a checklist application to solve finishing projects for adhd minds

//The first entity is a goal: This is a finished picture of what is to be done
pub trait Repository {
    fn create(&mut self, goal: GoalEntity) -> Result<(), String>;
    fn get_all(&self) -> Result<Vec<GoalEntity>, String>;
    fn get_by_id(&self, id: u32) -> Result<GoalEntity, String>;
    fn update(&mut self, id: u32, goal: GoalEntity) -> Result<(), String>;
    fn delete(&mut self, id: u32) -> Result<(), String>;
}

impl Clone for GoalEntity {
    fn clone(&self) -> Self {
        GoalEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            finish_date: self.finish_date.clone(),
            tags: self.tags.clone(),
        }
    }
}

pub struct GoalEntity {
    name: String,
    description: String,
    finish_date: String,
    tags: Vec<String>,
}

impl GoalEntity {
    pub fn new(name: String, description: String, finish_date: String, tags: Vec<String>) -> GoalEntity {
        GoalEntity {
            name: name,
            description: description,
            finish_date: finish_date,
            tags: tags,
        }
    }
}


pub struct add_goal_use_case {
    goal_entity: GoalEntity,
}

impl  add_goal_use_case {
    pub fn execute(goal_entity: GoalEntity, repo: &mut dyn Repository) -> Result<(), String>{
        match repo.create(goal_entity) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

pub struct GoalSqliteRepository {
    db: rusqlite::Connection,
}

impl GoalSqliteRepository {
    pub fn new(db: rusqlite::Connection) -> GoalSqliteRepository {
        GoalSqliteRepository {
            db: db,
        }
    }

    pub fn create_table(&mut self) -> Result<(), String> {
        let mut stmt = match self.db.prepare("CREATE TABLE IF NOT EXISTS goals (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, description TEXT, finish_date TEXT, tags TEXT)") {
            Ok(stmt) => stmt,
            Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
        };
        match stmt.execute([]) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        };
        Ok(())
    }

}

impl Repository for GoalSqliteRepository {
    fn create(&mut self, goal: GoalEntity) -> Result<(), String> {
        let name = &goal.name;
        let description = &goal.description;
        let finish_date = &goal.finish_date;
        let tags = &goal.tags;

        match self.db.execute(
            "INSERT INTO goals (name, description, finish_date, tags) VALUES (?, ?, ?, ?)",
            (name, description, finish_date, tags.join(",")),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }   

    fn get_all(&self) -> Result<Vec<GoalEntity>, String> {
        todo!()
    }

    fn get_by_id(&self, id: u32) -> Result<GoalEntity, String> {
        todo!()
    }

    fn update(&mut self, id: u32, goal: GoalEntity) -> Result<(), String> {
        todo!()
    }

    fn delete(&mut self, id: u32) -> Result<(), String> {
        todo!()
    }
}


pub struct controller {
    repo: GoalSqliteRepository,
    use_case: add_goal_use_case,
}

impl controller {
    pub fn init_repo(&mut self) -> Result<(), String> {
        self.repo = match rusqlite::Connection::open("goals.db") {
            Ok(db) => GoalSqliteRepository::new(db),
            Err(e) => return Err(format!("Failed to open database: {}", e)),
        };
        Ok(())
    }
    pub fn create_table(&mut self) -> Result<(), String> {
        match self.repo.create_table() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
            
        }
    }

    pub fn add_goal(&mut self, goal: GoalEntity) -> Result<(), String> {
        self.use_case = add_goal_use_case {
            goal_entity: goal,
        };
        match add_goal_use_case::execute(self.use_case.goal_entity.clone(), &mut self.repo) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}


//This is the main function that will be called when the program is run
fn main() {
    let mut controller = controller {
        repo: GoalSqliteRepository::new(rusqlite::Connection::open_in_memory().unwrap()),
        use_case: add_goal_use_case {
            goal_entity: GoalEntity::new(
                "Test Goal".to_string(),
                "This is a test goal".to_string(),
                "2024-01-01".to_string(),
                vec!["test".to_string()],
            ),
        },
    };
    controller.init_repo();
    controller.create_table();

    controller.add_goal(controller.use_case.goal_entity.clone());

}
