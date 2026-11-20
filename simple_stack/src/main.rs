#[derive(Clone,Debug)]
enum Exercise36State{Idle,Running,Finished}

struct Exercise36Record{id:usize,label:String,score:i32,state:Exercise36State}

impl Exercise36Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_36_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+36)).sum()
}

fn main(){
    println!("Exercise 36: simple_stack");
    let records=[
        Exercise36Record{id:1,label:"Ada".into(),score:98,state:Exercise36State::Finished},
        Exercise36Record{id:2,label:"Linus".into(),score:86,state:Exercise36State::Running},
        Exercise36Record{id:3,label:"Grace".into(),score:73,state:Exercise36State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_36_operation(&[12,7,19,3,25]));
}

const EXERCISE_36_27:i32=36;
const EXERCISE_36_29:i32=36;
const EXERCISE_36_30:i32=36;
const EXERCISE_36_31:i32=36;
const EXERCISE_36_32:i32=36;
const EXERCISE_36_33:i32=36;
const EXERCISE_36_34:i32=36;
const EXERCISE_36_35:i32=36;
const EXERCISE_36_36:i32=36;
const EXERCISE_36_37:i32=36;
const EXERCISE_36_38:i32=36;
const EXERCISE_36_39:i32=36;
const EXERCISE_36_40:i32=36;
const EXERCISE_36_41:i32=36;
const EXERCISE_36_42:i32=36;
const EXERCISE_36_43:i32=36;
const EXERCISE_36_44:i32=36;
const EXERCISE_36_45:i32=36;
const EXERCISE_36_46:i32=36;
const EXERCISE_36_47:i32=36;
const EXERCISE_36_48:i32=36;
const EXERCISE_36_49:i32=36;
const EXERCISE_36_50:i32=36;
const EXERCISE_36_51:i32=36;
const EXERCISE_36_52:i32=36;
const EXERCISE_36_53:i32=36;
const EXERCISE_36_54:i32=36;
const EXERCISE_36_55:i32=36;
const EXERCISE_36_56:i32=36;
const EXERCISE_36_57:i32=36;
const EXERCISE_36_58:i32=36;
const EXERCISE_36_59:i32=36;
const EXERCISE_36_60:i32=36;
const EXERCISE_36_61:i32=36;
const EXERCISE_36_62:i32=36;
const EXERCISE_36_63:i32=36;
const EXERCISE_36_64:i32=36;
const EXERCISE_36_65:i32=36;
const EXERCISE_36_66:i32=36;
const EXERCISE_36_67:i32=36;
const EXERCISE_36_68:i32=36;
const EXERCISE_36_69:i32=36;
const EXERCISE_36_70:i32=36;
const EXERCISE_36_71:i32=36;
const EXERCISE_36_72:i32=36;
const EXERCISE_36_73:i32=36;
const EXERCISE_36_74:i32=36;
const EXERCISE_36_75:i32=36;
const EXERCISE_36_76:i32=36;
const EXERCISE_36_77:i32=36;
const EXERCISE_36_78:i32=36;
const EXERCISE_36_79:i32=36;
