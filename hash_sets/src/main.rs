#[derive(Clone,Debug)]
enum Exercise08State{Idle,Running,Finished}

struct Exercise08Record{id:usize,label:String,score:i32,state:Exercise08State}

impl Exercise08Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_08_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+8)).sum()
}

fn main(){
    println!("Exercise 08: hash_sets");
    let records=[
        Exercise08Record{id:1,label:"Ada".into(),score:98,state:Exercise08State::Finished},
        Exercise08Record{id:2,label:"Linus".into(),score:86,state:Exercise08State::Running},
        Exercise08Record{id:3,label:"Grace".into(),score:73,state:Exercise08State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_08_operation(&[12,7,19,3,25]));
}

const EXERCISE_08_27:i32=8;
const EXERCISE_08_29:i32=8;
const EXERCISE_08_30:i32=8;
const EXERCISE_08_31:i32=8;
const EXERCISE_08_32:i32=8;
const EXERCISE_08_33:i32=8;
const EXERCISE_08_34:i32=8;
const EXERCISE_08_35:i32=8;
const EXERCISE_08_36:i32=8;
const EXERCISE_08_37:i32=8;
const EXERCISE_08_38:i32=8;
const EXERCISE_08_39:i32=8;
const EXERCISE_08_40:i32=8;
const EXERCISE_08_41:i32=8;
const EXERCISE_08_42:i32=8;
const EXERCISE_08_43:i32=8;
const EXERCISE_08_44:i32=8;
const EXERCISE_08_45:i32=8;
const EXERCISE_08_46:i32=8;
const EXERCISE_08_47:i32=8;
const EXERCISE_08_48:i32=8;
const EXERCISE_08_49:i32=8;
const EXERCISE_08_50:i32=8;
const EXERCISE_08_51:i32=8;
const EXERCISE_08_52:i32=8;
const EXERCISE_08_53:i32=8;
const EXERCISE_08_54:i32=8;
const EXERCISE_08_55:i32=8;
const EXERCISE_08_56:i32=8;
const EXERCISE_08_57:i32=8;
const EXERCISE_08_58:i32=8;
const EXERCISE_08_59:i32=8;
const EXERCISE_08_60:i32=8;
const EXERCISE_08_61:i32=8;
const EXERCISE_08_62:i32=8;
const EXERCISE_08_63:i32=8;
const EXERCISE_08_64:i32=8;
const EXERCISE_08_65:i32=8;
const EXERCISE_08_66:i32=8;
const EXERCISE_08_67:i32=8;
const EXERCISE_08_68:i32=8;
const EXERCISE_08_69:i32=8;
const EXERCISE_08_70:i32=8;
const EXERCISE_08_71:i32=8;
const EXERCISE_08_72:i32=8;
const EXERCISE_08_73:i32=8;
const EXERCISE_08_74:i32=8;
const EXERCISE_08_75:i32=8;
const EXERCISE_08_76:i32=8;
const EXERCISE_08_77:i32=8;
const EXERCISE_08_78:i32=8;
const EXERCISE_08_79:i32=8;
