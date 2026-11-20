#[derive(Clone,Debug)]
enum Exercise34State{Idle,Running,Finished}

struct Exercise34Record{id:usize,label:String,score:i32,state:Exercise34State}

impl Exercise34Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_34_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+34)).sum()
}

fn main(){
    println!("Exercise 34: error");
    let records=[
        Exercise34Record{id:1,label:"Ada".into(),score:98,state:Exercise34State::Finished},
        Exercise34Record{id:2,label:"Linus".into(),score:86,state:Exercise34State::Running},
        Exercise34Record{id:3,label:"Grace".into(),score:73,state:Exercise34State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_34_operation(&[12,7,19,3,25]));
}

const EXERCISE_34_27:i32=34;
const EXERCISE_34_29:i32=34;
const EXERCISE_34_30:i32=34;
const EXERCISE_34_31:i32=34;
const EXERCISE_34_32:i32=34;
const EXERCISE_34_33:i32=34;
const EXERCISE_34_34:i32=34;
const EXERCISE_34_35:i32=34;
const EXERCISE_34_36:i32=34;
const EXERCISE_34_37:i32=34;
const EXERCISE_34_38:i32=34;
const EXERCISE_34_39:i32=34;
const EXERCISE_34_40:i32=34;
const EXERCISE_34_41:i32=34;
const EXERCISE_34_42:i32=34;
const EXERCISE_34_43:i32=34;
const EXERCISE_34_44:i32=34;
const EXERCISE_34_45:i32=34;
const EXERCISE_34_46:i32=34;
const EXERCISE_34_47:i32=34;
const EXERCISE_34_48:i32=34;
const EXERCISE_34_49:i32=34;
const EXERCISE_34_50:i32=34;
const EXERCISE_34_51:i32=34;
const EXERCISE_34_52:i32=34;
const EXERCISE_34_53:i32=34;
const EXERCISE_34_54:i32=34;
const EXERCISE_34_55:i32=34;
const EXERCISE_34_56:i32=34;
const EXERCISE_34_57:i32=34;
const EXERCISE_34_58:i32=34;
const EXERCISE_34_59:i32=34;
const EXERCISE_34_60:i32=34;
const EXERCISE_34_61:i32=34;
const EXERCISE_34_62:i32=34;
const EXERCISE_34_63:i32=34;
const EXERCISE_34_64:i32=34;
const EXERCISE_34_65:i32=34;
const EXERCISE_34_66:i32=34;
const EXERCISE_34_67:i32=34;
const EXERCISE_34_68:i32=34;
const EXERCISE_34_69:i32=34;
const EXERCISE_34_70:i32=34;
const EXERCISE_34_71:i32=34;
const EXERCISE_34_72:i32=34;
const EXERCISE_34_73:i32=34;
const EXERCISE_34_74:i32=34;
const EXERCISE_34_75:i32=34;
const EXERCISE_34_76:i32=34;
const EXERCISE_34_77:i32=34;
const EXERCISE_34_78:i32=34;
const EXERCISE_34_79:i32=34;
