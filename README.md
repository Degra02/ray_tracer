# Ray Tracer
Porting of a ray tracer in Rust.

In order to use the program, either clone the repo and use 
```
cargo run
```  
or download the latest release and create a **data** folder in the same directory where you will execute the program.  

The renders will appear in the data directory under the name **XXXX.ppm**, where with a single image rendered it will always be 0000.ppm.

## World initialization
Use the provided world.json as a template.
To add a new sphere, add a new element to the vector in the json:
```
{
        "center":{
            "e":[1.1,0.0,-4.0]
            },
        "radius":3.5,
        "material":{
            "Metal":{
                "albedo":{
                    "e":[1.0,0.5,1.0]
                    },
                "fuzz": 0.5
            }
        }
    }
```  

>The example above represents a Sphere with **center** at __[1.1, 0.0, -4.0]__, with a **radius** of __3.5__ and a **Metal material** with a certain **albedo** and **fuzziness**


## Static image rendering
![image](./image.jpeg)

## Video Render (multiple static images + ffmpeg)
![render](./data/render_62.gif)

