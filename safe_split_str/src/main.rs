#[derive(Clone,Debug)]
enum Exercise37State{Idle,Running,Finished}

struct Exercise37Record{id:usize,label:String,score:i32,state:Exercise37State}

impl Exercise37Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_37_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+37)).sum()
}

fn main(){
    println!("Exercise 37: safe_split_str");
    let records=[
        Exercise37Record{id:1,label:"Ada".into(),score:98,state:Exercise37State::Finished},
        Exercise37Record{id:2,label:"Linus".into(),score:86,state:Exercise37State::Running},
        Exercise37Record{id:3,label:"Grace".into(),score:73,state:Exercise37State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_37_operation(&[12,7,19,3,25]));
}

const EXERCISE_37_27:i32=37;
const EXERCISE_37_29:i32=37;
const EXERCISE_37_30:i32=37;
const EXERCISE_37_31:i32=37;
const EXERCISE_37_32:i32=37;
const EXERCISE_37_33:i32=37;
const EXERCISE_37_34:i32=37;
const EXERCISE_37_35:i32=37;
const EXERCISE_37_36:i32=37;
const EXERCISE_37_37:i32=37;
const EXERCISE_37_38:i32=37;
const EXERCISE_37_39:i32=37;
const EXERCISE_37_40:i32=37;
const EXERCISE_37_41:i32=37;
const EXERCISE_37_42:i32=37;
const EXERCISE_37_43:i32=37;
const EXERCISE_37_44:i32=37;
const EXERCISE_37_45:i32=37;
const EXERCISE_37_46:i32=37;
const EXERCISE_37_47:i32=37;
const EXERCISE_37_48:i32=37;
const EXERCISE_37_49:i32=37;
const EXERCISE_37_50:i32=37;
const EXERCISE_37_51:i32=37;
const EXERCISE_37_52:i32=37;
const EXERCISE_37_53:i32=37;
const EXERCISE_37_54:i32=37;
const EXERCISE_37_55:i32=37;
const EXERCISE_37_56:i32=37;
const EXERCISE_37_57:i32=37;
const EXERCISE_37_58:i32=37;
const EXERCISE_37_59:i32=37;
const EXERCISE_37_60:i32=37;
const EXERCISE_37_61:i32=37;
const EXERCISE_37_62:i32=37;
const EXERCISE_37_63:i32=37;
const EXERCISE_37_64:i32=37;
const EXERCISE_37_65:i32=37;
const EXERCISE_37_66:i32=37;
const EXERCISE_37_67:i32=37;
const EXERCISE_37_68:i32=37;
const EXERCISE_37_69:i32=37;
const EXERCISE_37_70:i32=37;
const EXERCISE_37_71:i32=37;
const EXERCISE_37_72:i32=37;
const EXERCISE_37_73:i32=37;
const EXERCISE_37_74:i32=37;
const EXERCISE_37_75:i32=37;
const EXERCISE_37_76:i32=37;
const EXERCISE_37_77:i32=37;
const EXERCISE_37_78:i32=37;
const EXERCISE_37_79:i32=37;
