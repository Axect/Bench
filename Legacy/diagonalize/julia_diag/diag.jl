using LinearAlgebra

function proj(u, v)
    uv = dot(u, v)
    uu = dot(u, u)
    return (uv / uu) .* u
end

function gram_schmidt(m)
    (r, c) = size(m)
    result = zeros(r, c)
    col_norm = zeros(c)
    result[:,1] = m[:,1]
    col_norm[1] = norm(result[:,1])
    for k=2:c
        vk = m[:,k]
        puv = zeros(length(vk))
        for j=1:k-1
            puv += proj(result[:,j], vk)
        end
        result[:,k] = vk .- puv
        col_norm[k] = norm(result[:,k])
    end

    for i=1:c
        result[:,i] = result[:,i] ./ col_norm[i]
    end
    return result
end


function main()        
    m = rand(100,100)
    for i=1:100
        for j=1:i-1
            m[i,j] = m[j,i]
        end
    end

    for i=1:500
        dm = gram_schmidt(m)
        m = dm' * m * dm
    end

    return m
end

println(main())