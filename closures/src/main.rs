#[derive(Clone,Debug)]
enum Exercise06State{Idle,Running,Finished}

struct Exercise06Record{id:usize,label:String,score:i32,state:Exercise06State}

impl Exercise06Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_06_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+6)).sum()
}

fn main(){
    println!("Exercise 06: closures");
    let records=[
        Exercise06Record{id:1,label:"Ada".into(),score:98,state:Exercise06State::Finished},
        Exercise06Record{id:2,label:"Linus".into(),score:86,state:Exercise06State::Running},
        Exercise06Record{id:3,label:"Grace".into(),score:73,state:Exercise06State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_06_operation(&[12,7,19,3,25]));
}

const EXERCISE_06_27:i32=6;
const EXERCISE_06_29:i32=6;
const EXERCISE_06_30:i32=6;
const EXERCISE_06_31:i32=6;
const EXERCISE_06_32:i32=6;
const EXERCISE_06_33:i32=6;
const EXERCISE_06_34:i32=6;
const EXERCISE_06_35:i32=6;
const EXERCISE_06_36:i32=6;
const EXERCISE_06_37:i32=6;
const EXERCISE_06_38:i32=6;
const EXERCISE_06_39:i32=6;
const EXERCISE_06_40:i32=6;
const EXERCISE_06_41:i32=6;
const EXERCISE_06_42:i32=6;
const EXERCISE_06_43:i32=6;
const EXERCISE_06_44:i32=6;
const EXERCISE_06_45:i32=6;
const EXERCISE_06_46:i32=6;
const EXERCISE_06_47:i32=6;
const EXERCISE_06_48:i32=6;
const EXERCISE_06_49:i32=6;
const EXERCISE_06_50:i32=6;
const EXERCISE_06_51:i32=6;
const EXERCISE_06_52:i32=6;
const EXERCISE_06_53:i32=6;
const EXERCISE_06_54:i32=6;
const EXERCISE_06_55:i32=6;
const EXERCISE_06_56:i32=6;
const EXERCISE_06_57:i32=6;
const EXERCISE_06_58:i32=6;
const EXERCISE_06_59:i32=6;
const EXERCISE_06_60:i32=6;
const EXERCISE_06_61:i32=6;
const EXERCISE_06_62:i32=6;
const EXERCISE_06_63:i32=6;
const EXERCISE_06_64:i32=6;
const EXERCISE_06_65:i32=6;
const EXERCISE_06_66:i32=6;
const EXERCISE_06_67:i32=6;
const EXERCISE_06_68:i32=6;
const EXERCISE_06_69:i32=6;
const EXERCISE_06_70:i32=6;
const EXERCISE_06_71:i32=6;
const EXERCISE_06_72:i32=6;
const EXERCISE_06_73:i32=6;
const EXERCISE_06_74:i32=6;
const EXERCISE_06_75:i32=6;
const EXERCISE_06_76:i32=6;
const EXERCISE_06_77:i32=6;
const EXERCISE_06_78:i32=6;
const EXERCISE_06_79:i32=6;
