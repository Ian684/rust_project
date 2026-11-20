#[derive(Clone,Debug)]
enum Exercise27State{Idle,Running,Finished}

struct Exercise27Record{id:usize,label:String,score:i32,state:Exercise27State}

impl Exercise27Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_27_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+27)).sum()
}

fn main(){
    println!("Exercise 27: vectors");
    let records=[
        Exercise27Record{id:1,label:"Ada".into(),score:98,state:Exercise27State::Finished},
        Exercise27Record{id:2,label:"Linus".into(),score:86,state:Exercise27State::Running},
        Exercise27Record{id:3,label:"Grace".into(),score:73,state:Exercise27State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_27_operation(&[12,7,19,3,25]));
}

const EXERCISE_27_27:i32=27;
const EXERCISE_27_29:i32=27;
const EXERCISE_27_30:i32=27;
const EXERCISE_27_31:i32=27;
const EXERCISE_27_32:i32=27;
const EXERCISE_27_33:i32=27;
const EXERCISE_27_34:i32=27;
const EXERCISE_27_35:i32=27;
const EXERCISE_27_36:i32=27;
const EXERCISE_27_37:i32=27;
const EXERCISE_27_38:i32=27;
const EXERCISE_27_39:i32=27;
const EXERCISE_27_40:i32=27;
const EXERCISE_27_41:i32=27;
const EXERCISE_27_42:i32=27;
const EXERCISE_27_43:i32=27;
const EXERCISE_27_44:i32=27;
const EXERCISE_27_45:i32=27;
const EXERCISE_27_46:i32=27;
const EXERCISE_27_47:i32=27;
const EXERCISE_27_48:i32=27;
const EXERCISE_27_49:i32=27;
const EXERCISE_27_50:i32=27;
const EXERCISE_27_51:i32=27;
const EXERCISE_27_52:i32=27;
const EXERCISE_27_53:i32=27;
const EXERCISE_27_54:i32=27;
const EXERCISE_27_55:i32=27;
const EXERCISE_27_56:i32=27;
const EXERCISE_27_57:i32=27;
const EXERCISE_27_58:i32=27;
const EXERCISE_27_59:i32=27;
const EXERCISE_27_60:i32=27;
const EXERCISE_27_61:i32=27;
const EXERCISE_27_62:i32=27;
const EXERCISE_27_63:i32=27;
const EXERCISE_27_64:i32=27;
const EXERCISE_27_65:i32=27;
const EXERCISE_27_66:i32=27;
const EXERCISE_27_67:i32=27;
const EXERCISE_27_68:i32=27;
const EXERCISE_27_69:i32=27;
const EXERCISE_27_70:i32=27;
const EXERCISE_27_71:i32=27;
const EXERCISE_27_72:i32=27;
const EXERCISE_27_73:i32=27;
const EXERCISE_27_74:i32=27;
const EXERCISE_27_75:i32=27;
const EXERCISE_27_76:i32=27;
const EXERCISE_27_77:i32=27;
const EXERCISE_27_78:i32=27;
const EXERCISE_27_79:i32=27;
