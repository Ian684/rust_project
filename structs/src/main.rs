#[derive(Clone,Debug)]
enum Exercise23State{Idle,Running,Finished}

struct Exercise23Record{id:usize,label:String,score:i32,state:Exercise23State}

impl Exercise23Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_23_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+23)).sum()
}

fn main(){
    println!("Exercise 23: structs");
    let records=[
        Exercise23Record{id:1,label:"Ada".into(),score:98,state:Exercise23State::Finished},
        Exercise23Record{id:2,label:"Linus".into(),score:86,state:Exercise23State::Running},
        Exercise23Record{id:3,label:"Grace".into(),score:73,state:Exercise23State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_23_operation(&[12,7,19,3,25]));
}

const EXERCISE_23_27:i32=23;
const EXERCISE_23_29:i32=23;
const EXERCISE_23_30:i32=23;
const EXERCISE_23_31:i32=23;
const EXERCISE_23_32:i32=23;
const EXERCISE_23_33:i32=23;
const EXERCISE_23_34:i32=23;
const EXERCISE_23_35:i32=23;
const EXERCISE_23_36:i32=23;
const EXERCISE_23_37:i32=23;
const EXERCISE_23_38:i32=23;
const EXERCISE_23_39:i32=23;
const EXERCISE_23_40:i32=23;
const EXERCISE_23_41:i32=23;
const EXERCISE_23_42:i32=23;
const EXERCISE_23_43:i32=23;
const EXERCISE_23_44:i32=23;
const EXERCISE_23_45:i32=23;
const EXERCISE_23_46:i32=23;
const EXERCISE_23_47:i32=23;
const EXERCISE_23_48:i32=23;
const EXERCISE_23_49:i32=23;
const EXERCISE_23_50:i32=23;
const EXERCISE_23_51:i32=23;
const EXERCISE_23_52:i32=23;
const EXERCISE_23_53:i32=23;
const EXERCISE_23_54:i32=23;
const EXERCISE_23_55:i32=23;
const EXERCISE_23_56:i32=23;
const EXERCISE_23_57:i32=23;
const EXERCISE_23_58:i32=23;
const EXERCISE_23_59:i32=23;
const EXERCISE_23_60:i32=23;
const EXERCISE_23_61:i32=23;
const EXERCISE_23_62:i32=23;
const EXERCISE_23_63:i32=23;
const EXERCISE_23_64:i32=23;
const EXERCISE_23_65:i32=23;
const EXERCISE_23_66:i32=23;
const EXERCISE_23_67:i32=23;
const EXERCISE_23_68:i32=23;
const EXERCISE_23_69:i32=23;
const EXERCISE_23_70:i32=23;
const EXERCISE_23_71:i32=23;
const EXERCISE_23_72:i32=23;
const EXERCISE_23_73:i32=23;
const EXERCISE_23_74:i32=23;
const EXERCISE_23_75:i32=23;
const EXERCISE_23_76:i32=23;
const EXERCISE_23_77:i32=23;
const EXERCISE_23_78:i32=23;
const EXERCISE_23_79:i32=23;
