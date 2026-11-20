#[derive(Clone,Debug)]
enum Exercise35State{Idle,Running,Finished}

struct Exercise35Record{id:usize,label:String,score:i32,state:Exercise35State}

impl Exercise35Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_35_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+35)).sum()
}

fn main(){
    println!("Exercise 35: file_finder");
    let records=[
        Exercise35Record{id:1,label:"Ada".into(),score:98,state:Exercise35State::Finished},
        Exercise35Record{id:2,label:"Linus".into(),score:86,state:Exercise35State::Running},
        Exercise35Record{id:3,label:"Grace".into(),score:73,state:Exercise35State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_35_operation(&[12,7,19,3,25]));
}

const EXERCISE_35_27:i32=35;
const EXERCISE_35_29:i32=35;
const EXERCISE_35_30:i32=35;
const EXERCISE_35_31:i32=35;
const EXERCISE_35_32:i32=35;
const EXERCISE_35_33:i32=35;
const EXERCISE_35_34:i32=35;
const EXERCISE_35_35:i32=35;
const EXERCISE_35_36:i32=35;
const EXERCISE_35_37:i32=35;
const EXERCISE_35_38:i32=35;
const EXERCISE_35_39:i32=35;
const EXERCISE_35_40:i32=35;
const EXERCISE_35_41:i32=35;
const EXERCISE_35_42:i32=35;
const EXERCISE_35_43:i32=35;
const EXERCISE_35_44:i32=35;
const EXERCISE_35_45:i32=35;
const EXERCISE_35_46:i32=35;
const EXERCISE_35_47:i32=35;
const EXERCISE_35_48:i32=35;
const EXERCISE_35_49:i32=35;
const EXERCISE_35_50:i32=35;
const EXERCISE_35_51:i32=35;
const EXERCISE_35_52:i32=35;
const EXERCISE_35_53:i32=35;
const EXERCISE_35_54:i32=35;
const EXERCISE_35_55:i32=35;
const EXERCISE_35_56:i32=35;
const EXERCISE_35_57:i32=35;
const EXERCISE_35_58:i32=35;
const EXERCISE_35_59:i32=35;
const EXERCISE_35_60:i32=35;
const EXERCISE_35_61:i32=35;
const EXERCISE_35_62:i32=35;
const EXERCISE_35_63:i32=35;
const EXERCISE_35_64:i32=35;
const EXERCISE_35_65:i32=35;
const EXERCISE_35_66:i32=35;
const EXERCISE_35_67:i32=35;
const EXERCISE_35_68:i32=35;
const EXERCISE_35_69:i32=35;
const EXERCISE_35_70:i32=35;
const EXERCISE_35_71:i32=35;
const EXERCISE_35_72:i32=35;
const EXERCISE_35_73:i32=35;
const EXERCISE_35_74:i32=35;
const EXERCISE_35_75:i32=35;
const EXERCISE_35_76:i32=35;
const EXERCISE_35_77:i32=35;
const EXERCISE_35_78:i32=35;
const EXERCISE_35_79:i32=35;
