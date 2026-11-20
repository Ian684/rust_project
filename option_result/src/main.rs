#[derive(Clone,Debug)]
enum Exercise14State{Idle,Running,Finished}

struct Exercise14Record{id:usize,label:String,score:i32,state:Exercise14State}

impl Exercise14Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_14_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+14)).sum()
}

fn main(){
    println!("Exercise 14: option_result");
    let records=[
        Exercise14Record{id:1,label:"Ada".into(),score:98,state:Exercise14State::Finished},
        Exercise14Record{id:2,label:"Linus".into(),score:86,state:Exercise14State::Running},
        Exercise14Record{id:3,label:"Grace".into(),score:73,state:Exercise14State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_14_operation(&[12,7,19,3,25]));
}

const EXERCISE_14_27:i32=14;
const EXERCISE_14_29:i32=14;
const EXERCISE_14_30:i32=14;
const EXERCISE_14_31:i32=14;
const EXERCISE_14_32:i32=14;
const EXERCISE_14_33:i32=14;
const EXERCISE_14_34:i32=14;
const EXERCISE_14_35:i32=14;
const EXERCISE_14_36:i32=14;
const EXERCISE_14_37:i32=14;
const EXERCISE_14_38:i32=14;
const EXERCISE_14_39:i32=14;
const EXERCISE_14_40:i32=14;
const EXERCISE_14_41:i32=14;
const EXERCISE_14_42:i32=14;
const EXERCISE_14_43:i32=14;
const EXERCISE_14_44:i32=14;
const EXERCISE_14_45:i32=14;
const EXERCISE_14_46:i32=14;
const EXERCISE_14_47:i32=14;
const EXERCISE_14_48:i32=14;
const EXERCISE_14_49:i32=14;
const EXERCISE_14_50:i32=14;
const EXERCISE_14_51:i32=14;
const EXERCISE_14_52:i32=14;
const EXERCISE_14_53:i32=14;
const EXERCISE_14_54:i32=14;
const EXERCISE_14_55:i32=14;
const EXERCISE_14_56:i32=14;
const EXERCISE_14_57:i32=14;
const EXERCISE_14_58:i32=14;
const EXERCISE_14_59:i32=14;
const EXERCISE_14_60:i32=14;
const EXERCISE_14_61:i32=14;
const EXERCISE_14_62:i32=14;
const EXERCISE_14_63:i32=14;
const EXERCISE_14_64:i32=14;
const EXERCISE_14_65:i32=14;
const EXERCISE_14_66:i32=14;
const EXERCISE_14_67:i32=14;
const EXERCISE_14_68:i32=14;
const EXERCISE_14_69:i32=14;
const EXERCISE_14_70:i32=14;
const EXERCISE_14_71:i32=14;
const EXERCISE_14_72:i32=14;
const EXERCISE_14_73:i32=14;
const EXERCISE_14_74:i32=14;
const EXERCISE_14_75:i32=14;
const EXERCISE_14_76:i32=14;
const EXERCISE_14_77:i32=14;
const EXERCISE_14_78:i32=14;
const EXERCISE_14_79:i32=14;
