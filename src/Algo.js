

let usernames = ["Tricer66", "TREXerT", "Veloc1COLEv", "stego95"]
// ["TREXerT", "Veloc1COLEv"]

export function findAllPalindromes(usernames) {
    let result = []
    for (let o of usernames) {
        let check = (function findLetter(){
            for (let i = 0; i < o.length; i++) 
            {
                if ((o[i]).toLowerCase() == (o[o.length -1 - i ]).toLowerCase())
                {
                    continue
                } 
                else 
                {
                    return false
                }  
            }
        })()
        if (check == false){
            continue
        } else {
            result.push(o)
        }
    }
    return result
}

console.log(findAllPalindromes(usernames));