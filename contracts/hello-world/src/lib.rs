#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, Symbol, String, Map
};

// Storage key
const STUDENT_GRADES: Symbol = symbol_short!("GRADES");

#[contracttype]
#[derive(Clone)]
pub struct Student {
    pub name: String,
    pub score: u32,
}

#[contract]
pub struct GradeContract;

#[contractimpl]
impl GradeContract {

    // 📌 Add or update a student's grade
    pub fn set_grade(env: Env, student_id: String, name: String, score: u32) {
        let mut grades: Map<String, Student> =
            env.storage().instance().get(&STUDENT_GRADES).unwrap_or(Map::new(&env));

        let student = Student { name, score };

        grades.set(student_id, student);

        env.storage().instance().set(&STUDENT_GRADES, &grades);
    }

    // 📌 Get a student's grade
    pub fn get_grade(env: Env, student_id: String) -> Option<Student> {
        let grades: Map<String, Student> =
            env.storage().instance().get(&STUDENT_GRADES).unwrap_or(Map::new(&env));

        grades.get(student_id)
    }

    // 📌 Remove a student
    pub fn remove_student(env: Env, student_id: String) {
        let mut grades: Map<String, Student> =
            env.storage().instance().get(&STUDENT_GRADES).unwrap_or(Map::new(&env));

        grades.remove(student_id);

        env.storage().instance().set(&STUDENT_GRADES, &grades);
    }

    // 📌 Get all students (optional)
    pub fn get_all(env: Env) -> Map<String, Student> {
        env.storage().instance().get(&STUDENT_GRADES).unwrap_or(Map::new(&env))
    }
}